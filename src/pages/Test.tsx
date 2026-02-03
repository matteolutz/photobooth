import { convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { appDataDir, join } from "@tauri-apps/api/path";
import { useEffect, useState } from "react";

const EVF_FILE = "evf.jpeg";

const Test = () => {
  const [image, setImage] = useState<string | null>(null);

  useEffect(() => {
    const unlisten = (async () => {
      const appData = await appDataDir();
      const path = await join(appData, EVF_FILE);
      const photoSrc = convertFileSrc(path);

      return listen("evf-update", () => {
        setImage(`${photoSrc}?${Date.now()}`);

        // console.log("got evf update");
      });
    })();

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return image !== null ? <img src={image} alt="EVF" /> : <div>Loading...</div>;
};

export default Test;
