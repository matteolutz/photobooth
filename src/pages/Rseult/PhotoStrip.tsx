import { FC, useCallback, useEffect, useRef, useState } from "react";
import { PhotoboothResult } from "../../types/result";

import kmgLogo from "../../assets/images/kmg.png";

type PhotoStripProps = {
  photos: PhotoboothResult["photos"];
  onImagesLoaded?: (container: HTMLDivElement) => void;
};

const PhotoStrip: FC<PhotoStripProps> = ({ photos, onImagesLoaded }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [loadedImages, setLoadedImages] = useState<number>(0);

  useEffect(() => {
    if (loadedImages !== photos.length) {
      return;
    }

    onImagesLoaded?.(containerRef.current!);
  }, [loadedImages, photos, containerRef]);

  const onImageLoad = useCallback(() => {
    setLoadedImages((prev) => prev + 1);
  }, []);

  return (
    <div
      id="photo-strip"
      className="bg-gray-800 p-3 rounded-lg shadow-lg w-48 md:w-56"
    >
      <div className="flex flex-col items-center space-y-3">
        {photos.map((photo, index) => (
          <div key={index} className="bg-white p-1 w-full">
            <img
              onLoad={onImageLoad}
              src={photo}
              alt={`Photo ${index + 1}`}
              className="photo-strip-img w-full h-auto object-cover"
            />
          </div>
        ))}
        <div className="flex flex-col items-center py-2">
          <img src={kmgLogo} className="h-10" />
          <p className="text-orange-400 text-xs font-medium">
            2026 - Was ich mal werden will
          </p>
        </div>
      </div>
    </div>
  );
};

export default PhotoStrip;
