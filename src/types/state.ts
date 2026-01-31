import { Photos } from "./result";
import { User } from "./user";

export type PhotoboothStateReady = {
  state: "ready";
};

export type PhotoboothStateCountdown = {
  state: "countdown";
  user: User;
};

export type PhotoboothStateResult = {
  state: "result";
  photos: Photos;
  user: User;
};

export type PhotoboothState =
  | PhotoboothStateReady
  | PhotoboothStateCountdown
  | PhotoboothStateResult;
