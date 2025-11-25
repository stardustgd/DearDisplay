"use client";

import html2canvas from "html2canvas";

export default function HtmlToImage() {
  async function captureAndSend() {
    const element = document.getElementById("capture-target");

    if (!element) return;

    const canvas = await html2canvas(element);

    const blob = await new Promise<Blob | null>((resolve) =>
      canvas.toBlob(resolve, "image/png"),
    );

    if (!blob) return;

    const formData = new FormData();
    formData.append("file", blob, "capture.png");

    await fetch("http://localhost:4000/api/display", {
      method: "POST",
      body: formData,
    });
  }

  return (
    <div className="flex flex-col gap-16">
      <div
        id="capture-target"
        className="flex flex-col justify-center items-center bg-white h-[480px] w-[800px]"
      >
        <h1 className="text-black">Hello world</h1>
        <p className="text-black">HTML to E-Ink??</p>
      </div>

      <button onClick={captureAndSend}>Convert and Upload</button>
    </div>
  );
}
