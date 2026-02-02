import { FC, useState } from "react";
import { AnimationItem } from "./context";

const AnimationLayer: FC<{
  item: AnimationItem | null;
  onDone: () => void;
}> = ({ item, onDone }) => {
  const [play, setPlay] = useState(false);

  if (!item) return null;

  const { from, to, content } = item;

  return (
    <div className="pointer-events-none fixed inset-0 z-50">
      <div
        className="absolute transition-all duration-500 ease-out"
        style={{
          top: play ? to.top : from.top,
          left: play ? to.left : from.left,
          width: play ? to.width : from.width,
          height: play ? to.height : from.height,
          transform: play ? "translate(0, 0)" : "translate(-50%, -50%)",
        }}
        onTransitionEnd={onDone}
        ref={() => {
          requestAnimationFrame(() => setPlay(true));
        }}
      >
        {content}
      </div>
    </div>
  );
};

export default AnimationLayer;
