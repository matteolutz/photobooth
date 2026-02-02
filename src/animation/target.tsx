import { FC, HTMLProps, useEffect, useRef } from "react";
import { useAnimationLayer } from "./context";

const AnimationTarget: FC<
  HTMLProps<HTMLDivElement> & { animationId: string }
> = ({ animationId, className, children, ...props }) => {
  const ref = useRef<HTMLDivElement>(null);
  const { registerTarget } = useAnimationLayer();

  useEffect(() => {
    if (ref.current) {
      registerTarget(animationId, ref.current);
    }
  }, []);

  return (
    <div
      ref={ref}
      {...props}
      className={
        (className ?? "") + " w-full h-full pointer-events-none opacity-0"
      }
    >
      {children}
    </div>
  );
};

export default AnimationTarget;
