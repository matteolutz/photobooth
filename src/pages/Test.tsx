import { useEffect, useRef, useState } from "react";
import { useAnimationLayer } from "../animation/context";
import PhotoStrip from "./Result/PhotoStrip";
import { toOptionalPhotos } from "../types/result";
import { appDataDir, join } from "@tauri-apps/api/path";

const Test = () => {
  const { animateTo } = useAnimationLayer();

  const [photos, setPhotos] = useState<string[]>([]);

  const testPhoto = "02-02-2026 19-01-26.jpeg";
  const testPhotoPath = useRef("");

  useEffect(() => {
    (async () => {
      const appData = await appDataDir();
      testPhotoPath.current = await join(appData, "camera", testPhoto);
    })();
  }, []);

  const test = () => {
    animateTo(
      `photostrip-${photos.length}`,
      {
        top: window.innerWidth / 50,
        left: window.innerHeight / 50,
        width: 200,
        height: 200,
      },
      <img
        src={testPhotoPath.current}
        className="photo-strip-img w-full h-auto object-cover"
      />,
      {
        onDone: () => setPhotos([...photos, testPhoto]),
      },
    );
  };

  return (
    <div>
      <PhotoStrip photos={toOptionalPhotos(photos)} />

      <button onClick={test}>test</button>
    </div>
  );
};

export default Test;
