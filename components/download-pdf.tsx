"use client";

export default function DownloadPdf() {
  return (
    <button
      type="button"
      onClick={() => window.print()}
      className="secondary-cta"
    >
      Download PDF
    </button>
  );
}
