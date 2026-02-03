import { FC, useEffect, useRef, useState } from "react";

const COUNTDOWN_DURATION = 3;

type NthPhotoOf = [number, number];

const Countdown: FC<{ onFinish: () => void; nPhoto: NthPhotoOf }> = ({
  onFinish,
  nPhoto: [currentPhoto, totalPhotos],
}) => {
  const [count, setCount] = useState(COUNTDOWN_DURATION);
  const prevPhoto = useRef(currentPhoto);

  useEffect(() => {
    if (prevPhoto.current !== currentPhoto) {
      setCount(COUNTDOWN_DURATION);
      prevPhoto.current = currentPhoto;
      return;
    }

    if (count <= 0) {
      onFinish();
      return;
    }

    const timer = setTimeout(() => {
      setCount(count - 1);
    }, 1000);

    return () => clearTimeout(timer);
  }, [count, onFinish, currentPhoto]);

  return (
    <div className="bg-white/80 backdrop-blur-sm rounded-3xl shadow-2xl p-8 md:p-12 text-center transform transition-all duration-500 hover:scale-105">
      <div className="flex flex-col items-center justify-center text-center">
        <p className="text-4xl text-gray-800 font-semibold mb-8 animate-pulse">
          {/*Macht Euch bereit!*/}
          Foto {currentPhoto} von {totalPhotos}
        </p>
        <div
          key={count}
          className="text-9xl font-bold text-white bg-orange-500 rounded-full w-48 h-48 flex items-center justify-center shadow-2xl animate-ping-once"
        >
          {count > 0 ? count : "📸"}
        </div>
        <style>{`
          @keyframes ping-once {
            0% { transform: scale(0.5); opacity: 0; }
            50% { transform: scale(1.1); opacity: 1; }
            100% { transform: scale(1); opacity: 1; }
          }
          .animate-ping-once {
            animation: ping-once 1s ease-out;
          }
        `}</style>
      </div>
    </div>
  );
};

export default Countdown;
