import { PhotoboothStateResult } from "./state";
import { User } from "./user";

export type Photos = [string, string, string, string];

export type PhotoboothResult = {
  photos: Photos;
  user: User;
};

export const resultFromState = (
  state: PhotoboothStateResult,
): PhotoboothResult => ({
  photos: state.photos,
  user: state.user,
});
