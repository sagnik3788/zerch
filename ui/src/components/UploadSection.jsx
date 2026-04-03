import { useState, useRef } from "react";
import "./UploadSection.css";

export default function UploadSection({ onUpload }) {
  const [isDragging, setIsDragging] = useState(false);
  const [isUploading, setIsUploading] = useState(false);
  const [error, setError] = useState(null);
  const fileInputRef = useRef(null);

  const handleDragOver = (e) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = () => {
    setIsDragging(false);
  };

  const handleDrop = (e) => {
    e.preventDefault();
    setIsDragging(false);
    const files = e.dataTransfer.files;
    if (files.length > 0) {
      processFile(files[0]);
    }
  };

  const handleFileSelect = (e) => {
    const files = e.target.files;
    if (files.length > 0) {
      processFile(files[0]);
    }
  };

  const processFile = async (file) => {
    setIsUploading(true);
    setError(null);

    try {
      const formData = new FormData();
      formData.append("file", file);

      const response = await fetch("http://localhost:8080/api/upload", {
        method: "POST",
        body: formData,
      });

      if (!response.ok) {
        throw new Error(`Upload failed: ${response.statusText}`);
      }

      const data = await response.json();

      if (data.success && data.logs) {
        onUpload(data.logs);
        setError(null);
      } else {
        throw new Error("Invalid response from server");
      }
    } catch (err) {
      console.error("Upload error:", err);
      setError(err.message || "Failed to upload file");
    } finally {
      setIsUploading(false);
    }
  };

  return (
    <div className="upload-section card">
      <div className="card-header">
        <h2 className="card-title">Upload Logs</h2>
        <svg
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="var(--accent-blue)"
          strokeWidth="2.5"
        >
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
          <polyline points="17 8 12 3 7 8" />
          <line x1="12" y1="3" x2="12" y2="15" />
        </svg>
      </div>
      <div
        className={`upload-zone ${isDragging ? "upload-zone--dragging" : ""} ${isUploading ? "upload-zone--uploading" : ""}`}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        onClick={() => !isUploading && fileInputRef.current?.click()}
      >
        <input
          type="file"
          ref={fileInputRef}
          onChange={handleFileSelect}
          style={{ display: "none" }}
          accept=".log,.txt,.json"
          disabled={isUploading}
        />
        <div className="upload-icon">
          {isUploading ? (
            <svg
              width="32"
              height="32"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              className="spinner"
            >
              <circle cx="12" cy="12" r="10" />
            </svg>
          ) : (
            <svg
              width="32"
              height="32"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.5"
            >
              <rect x="3" y="3" width="18" height="18" rx="2" />
              <path d="M12 8v8" />
              <path d="M8 12h8" />
            </svg>
          )}
        </div>
        <p className="upload-text">
          {isUploading ? "Processing logs..." : "Drag & drop log files"}
        </p>
        <p className="upload-subtext">
          {isUploading
            ? "Embedding and indexing in progress"
            : "Supports .log, .txt, .json"}
        </p>
      </div>
      {error && (
        <div className="upload-error">
          <span className="error-icon">⚠️</span>
          <span className="error-message">{error}</span>
        </div>
      )}
    </div>
  );
}
