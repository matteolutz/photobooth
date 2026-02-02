import { Photos } from "../types/result";
import PhotoStrip from "./Result/PhotoStrip";
import * as htmlToImage from "html-to-image";

const Test = () => {
  const photos = [
    "02-02-2026 18-49-04.jpeg", "02-02-2026 18-49-10.jpeg", "02-02-2026 18-49-16.jpeg", "02-02-2026 18-49-23.jpeg",
  ] as Photos;

  const onImagesLoaded = () => {
    htmlToImage
      .toPng(document.getElementById("photo-strip")!, { pixelRatio: 4 })
      .then(async (dataUrl) => {
        console.log("downloading");
        const a = document.createElement("a");
        a.download = "test.png";
        a.href = dataUrl;
        a.click();
      })
  };

  return <PhotoStrip photos={photos} onImagesLoaded={onImagesLoaded} />
};

export default Test;
