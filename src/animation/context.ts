import { createContext, ReactNode, useContext } from "react";

export type Rect = {
  top: number;
  left: number;
  width: number;
  height: number;
};

export type AnimationItem = {
  id: string;
  content: ReactNode;
  from: Rect;
  to: Rect;
  onDone?: () => void;
};

export type AnimationContextType = {
  registerTarget: (id: string, el: HTMLElement) => void;
  animateTo: (
    id: string,
    from: Rect,
    content: ReactNode,
    options?: { onDone?: () => void },
  ) => void;
};

export const AnimationContext = createContext<AnimationContextType | null>(
  null,
);

export const useAnimationLayer = () => {
  const ctx = useContext(AnimationContext);
  if (!ctx)
    throw new Error("useAnimationLayer must be called from inside a provider");
  return ctx;
};
