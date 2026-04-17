#!/usr/bin/env python3
"""
Drain3 wrapper for Zerch - handles log template mining and parameter extraction.
Run this script with stdin input, or import as a module.
"""

import json
import sys
from typing import Any, Dict, List, Optional

try:
    from drain3 import TemplateMiner
    from drain3.template_miner_config import TemplateMinerConfig
except ImportError:
    print("Error: drain3 not installed. Run: pip install drain3", file=sys.stderr)
    sys.exit(1)


class ZerchDrain3:
    def __init__(self, config_path: Optional[str] = None):
        if config_path and config_path.endswith(".ini"):
            config = TemplateMinerConfig()
            config.load(config_path)
        else:
            config = TemplateMinerConfig()

            config.mask_prefix = "<:"
            config.mask_suffix = ":>"
            config.drain_sim_th = 0.4
            config.drain_depth = 4
            config.drain_max_children = 100
            config.drain_max_clusters = 1000

            config.parametrize_numeric_tokens = True

            config.drain_extra_delimiters = []

            from drain3.masking import RegexMaskingInstruction

            config.masking_instructions = [
                # Drain3 recommended patterns for general server logs
                # Hex numbers (0x prefix) - must come before NUM
                RegexMaskingInstruction(r"(0x[a-fA-F0-9]+)", "HEX"),
                # Milliseconds/ms durations
                RegexMaskingInstruction(r"(\d+)ms", "MS"),
                # Percentages - must come before NUM
                RegexMaskingInstruction(r"(\d+)%", "PCT"),
                # HTTP status codes - must come before NUM
                RegexMaskingInstruction(
                    r"\b(200|201|204|301|302|400|401|403|404|500|502|503|504)\b",
                    "STATUS",
                ),
                # IP addresses - using lookbehind/lookahead to ensure standalone IP
                RegexMaskingInstruction(
                    r"((?<=[^A-Za-z0-9])|^)(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})((?=[^A-Za-z0-9])|$)",
                    "IP",
                ),
                # UUIDs - must come before NUM
                RegexMaskingInstruction(
                    r"([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})",
                    "UUID",
                ),
                # Numbers (integers) - with optional sign - must be last
                RegexMaskingInstruction(
                    r"((?<=[^A-Za-z0-9])|^)([\-\+]?\d+)((?=[^A-Za-z0-9])|$)", "NUM"
                ),
            ]

        self.miner = TemplateMiner(config=config)
        self.template_cache: Dict[str, str] = {}

    def process_log(self, log_line: str) -> Dict[str, Any]:
        line = log_line.strip()
        if not line:
            return {"error": "Empty line"}

        result = self.miner.add_log_message(line)

        cluster_id = result.get("cluster_id")
        template = result.get("template_mined")
        change_type = result.get("change_type")
        cluster_size = result.get("cluster_size")

        params = self.miner.extract_parameters(template, log_line)

        return {
            "raw_log": line,
            "template": template,
            "cluster_id": cluster_id,
            "cluster_size": cluster_size,
            "change_type": change_type,
            "params": [{"value": p.value, "mask_name": p.mask_name} for p in params],
        }

    def process_logs(self, logs: List[str]) -> List[Dict[str, Any]]:
        results = []
        for log in logs:
            result = self.process_log(log)
            results.append(result)
        return results

    def get_template(self, log_line: str) -> Optional[str]:
        line = log_line.strip()
        if not line:
            return None

        result = self.miner.match(line)
        if result:
            return result.get("template_mined")
        return None

    def get_or_create_template(self, log_line: str) -> str:
        result = self.process_log(log_line)
        return result.get("template", log_line)

    def get_state(self) -> Dict[str, Any]:
        return {
            "cluster_count": len(self.miner.drain.id_to_cluster),
            "templates": [
                {
                    "cluster_id": cid,
                    "template": cluster.log_template,
                    "size": cluster.size,
                }
                for cid, cluster in self.miner.drain.id_to_cluster.items()
            ],
        }

    def save_state(self, path: str):
        self.miner.save_state(path)

    def load_state(self, path: str):
        self.miner.load_state(path)


def main():
    import argparse

    parser = argparse.ArgumentParser(description="Drain3 wrapper for Zerch")
    parser.add_argument("--config", type=str, help="Path to drain3.ini config")
    parser.add_argument("--file", type=str, help="Process a log file line by line")
    parser.add_argument("--single", type=str, help="Process a single log line")
    parser.add_argument(
        "--template-only", action="store_true", help="Only return template"
    )
    args = parser.parse_args()

    drain = ZerchDrain3(config_path=args.config)

    if args.single:
        result = drain.process_log(args.single)
        if args.template_only:
            print(result.get("template", ""))
        else:
            print(json.dumps(result, indent=2))
        return

    if args.file:
        with open(args.file, "r") as f:
            logs = [line.strip() for line in f if line.strip()]

        results = []
        for log in logs:
            result = drain.process_log(log)
            results.append(result)

        for r in results:
            if args.template_only:
                print(r.get("template", ""))
            else:
                print(json.dumps(r))
        return

    for line in sys.stdin:
        line = line.strip()
        if not line:
            continue
        result = drain.process_log(line)
        if args.template_only:
            print(result.get("template", ""))
        else:
            print(json.dumps(result))


if __name__ == "__main__":
    main()
