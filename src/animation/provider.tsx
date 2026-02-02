import { FC, PropsWithChildren, ReactNode, useRef, useState } from "react";
import { AnimationContext, AnimationItem, Rect } from "./context";
import AnimationLayer from "./layer";

const AnimationLayerProvider: FC<PropsWithChildren> = ({ children }) => {
  const targets = useRef<Map<string, HTMLElement>>(new Map());
  const [active, setActive] = useState<AnimationItem | null>(null);

  const registerTarget = (id: string, element: HTMLElement) => {
    targets.current.set(id, element);
  };

  const animateTo = (
    id: string,
    from: Rect,
    content: ReactNode,
    options?: { onDone?: () => void },
  ) => {
    const target = targets.current.get(id);
    if (!target) return;

    const targetRect = target.getBoundingClientRect();

    setActive({
      id,
      content,
      from,
      to: targetRect,
      onDone: options?.onDone,
    });
  };

  return (
    <AnimationContext.Provider value={{ registerTarget, animateTo }}>
      {children}
      <AnimationLayer
        item={active}
        onDone={() => {
          active?.onDone?.();
          setActive(null);
        }}
      />
    </AnimationContext.Provider>
  );
};

export default AnimationLayerProvider;
