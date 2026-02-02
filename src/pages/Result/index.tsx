import { FC, useEffect, useState } from "react";
import { PhotoboothResult } from "../../types/result";
import PhotoStrip from "./PhotoStrip";
import { invoke } from "@tauri-apps/api/core";
import { domToBlob } from "modern-screenshot";

const Result: FC<{ result: PhotoboothResult; onReset: () => void }> = ({
  result,
  onReset,
}) => {
  const [canRestart, setCanRestart] = useState(false);

  useEffect(() => {
    const timeout = setTimeout(() => onReset(), 60_000);
    return () => clearTimeout(timeout);
  }, []);

  const onImagesLoaded = () => {
      domToBlob(document.getElementById("photo-strip")!, { scale: 4 })
      .then(async (blob) => {
        const arrayBuffer = await blob?.arrayBuffer();
        const bytes = Array.from(new Uint8Array(arrayBuffer!));

        return invoke("send_mail", {
          mailAddress: result.user.email,
          photos: result.photos,
          stripImage: bytes,
        });
      })
      .then(() => setCanRestart(true));
  };

  return (
    <div className="bg-white/80 backdrop-blur-sm rounded-3xl shadow-2xl p-6 md:p-10 text-center transform transition-all duration-500">
      <h1 className="text-4xl md:text-5xl font-bold text-gray-800 mb-4">
        Großartig!
      </h1>
      <p className="text-lg text-gray-600 mb-6">
        Hier sind deine wunderbaren Fotos. Eine Kopie wird an{" "}
        <span className="font-semibold text-orange-600">
          {result.user.email}
        </span>{" "}
        gesendet.
      </p>

      <div className="flex justify-center mb-8">
        <PhotoStrip onImagesLoaded={onImagesLoaded} photos={result.photos} />
      </div>

      <button
        disabled={!canRestart}
        onClick={onReset}
        className="w-full max-w-xs mx-auto flex justify-center py-3 px-6 border border-transparent rounded-full shadow-lg text-lg font-bold text-white bg-orange-500 hover:bg-orange-600 disabled:bg-gray-300 focus:outline-none focus:ring-4 focus:ring-orange-400 transform hover:-translate-y-1 transition-all duration-300"
      >
        {canRestart ? "Neu starten" : "Fotos werden gesendet..."}
      </button>
    </div>
  );
};

export default Result;
