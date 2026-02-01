import {
  Dispatch,
  RefObject,
  SetStateAction,
  useEffect,
  useRef,
  useState,
} from "react";

export const useRefState = <T>(
  initialValue: T | (() => T),
): [T, Dispatch<SetStateAction<T>>, RefObject<T>] => {
  const [state, setState] = useState(initialValue);
  const ref = useRef<T>(state);

  useEffect(() => {
    ref.current = state;
  }, [state]);

  return [state, setState, ref];
};
