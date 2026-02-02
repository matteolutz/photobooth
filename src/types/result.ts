import { PhotoboothStateResult } from "./state";
import { User } from "./user";

export type Photos = [string, string, string, string];
export type OptionalPhotos = [
  string | null,
  string | null,
  string | null,
  string | null,
];

export const toOptionalPhotos = (photos: Photos[0][]): OptionalPhotos => [
  photos[0] ?? null,
  photos[1] ?? null,
  photos[2] ?? null,
  photos[3] ?? null,
];

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
