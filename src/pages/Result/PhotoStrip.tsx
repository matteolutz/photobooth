import { FC, useCallback, useEffect, useRef, useState } from "react";
import { OptionalPhotos } from "../../types/result";

import kmgLogo from "../../assets/images/kmg.png";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import AnimationTarget from "../../animation/target";

type PhotoStripProps = {
  photos: OptionalPhotos;
  onImagesLoaded?: (container: HTMLDivElement) => void;
};

const PhotoStrip: FC<PhotoStripProps> = ({ photos, onImagesLoaded }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const [loadedImages, setLoadedImages] = useState<number>(0);

  const [photoSrc, setPhotoSrc] = useState<(string | null)[] | null>(null);

  useEffect(() => {
    (async () => {
      const appData = await appDataDir();

      const photoSrc = await Promise.all(
        photos.map(async (photo) => {
          if (photo === null) return null;
          const photoPath = await join(appData, "camera", photo);
          return convertFileSrc(photoPath);
        }),
      );

      setPhotoSrc(photoSrc);
    })();
  }, [photos]);

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
        {photoSrc != null &&
          photoSrc.map((photo, index) => (
            <div key={index} className="bg-white p-1 w-full">
              {photo !== null ? (
                <img
                  onLoad={onImageLoad}
                  src={photo}
                  alt={`Photo ${index + 1}`}
                  className="photo-strip-img w-full h-auto object-cover"
                />
              ) : (
                <div className="w-full aspect-square">
                  <AnimationTarget animationId={`photostrip-${index}`} />
                </div>
              )}
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
