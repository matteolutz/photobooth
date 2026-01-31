import { User } from "./user";

export type PhotoboothStateReady = {
  state: "ready";
};

export type PhotoboothStateCountdown = {
  state: "countdown";
  user: User;
};

export type PhotoboothStateResults = {
  state: "results";
  results: [unknown, unknown, unknown, unknown];
  user: User;
};

export type PhotoboothState =
  | PhotoboothStateReady
  | PhotoboothStateCountdown
  | PhotoboothStateResults;
