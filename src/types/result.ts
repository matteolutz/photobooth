import { PhotoboothStateResult } from "./state";
import { User } from "./user";

export type Photos = [string, string, string, string];

export const allPhotosTaken = (photos: Photos[0][]): photos is Photos => {
  return photos.length >= 4;
};

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
