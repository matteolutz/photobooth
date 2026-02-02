#include <stdbool.h>

// this fixes a bug in EDSDKTypes.h
#define __int64 long long
#include "include/EDSDK.h"

#include <stdio.h>

int main(void) {
  EdsError err = EDS_ERR_OK;
  EdsCameraListRef cameraList = NULL;

  printf("sizeof(EdsBaseRef): %lu\n", sizeof(EdsBaseRef));

  err = EdsInitializeSDK();
  if (err != EDS_ERR_OK) {
    printf("Failed to load EDSDK\n");
    return 1;
  }

  err = EdsGetCameraList(&cameraList);
  if (err != EDS_ERR_OK) {
    printf("Failed to get camera list\n");
    return 1;
  }

  EdsUInt32 cameraCount = 0;
  err = EdsGetChildCount(cameraList, &cameraCount);
  if (err != EDS_ERR_OK) {
    printf("Failed to get number of cameras\n");
    return 1;
  }

  printf("Found %lu cameras\n", cameraCount);

  if (cameraCount == 0) {
    return 0;
  }

  printf("Using first found camera\n");

  EdsCameraRef camera = NULL;
  err = EdsGetChildAtIndex(cameraList, 0, &camera);
  if (err != EDS_ERR_OK) {
    printf("Failed to get camera\n");
    return 1;
  }

  EdsDeviceInfo info;
  err = EdsGetDeviceInfo(camera, &info);
  if (err != EDS_ERR_OK) {
    printf("Failed to get camera device info\n");
    return 1;
  }

  printf("Name: %s\n", info.szPortName);
  printf("Description: %s\n", info.szDeviceDescription);

  err = EdsOpenSession(camera);
  if (err != EDS_ERR_OK) {
    printf("Failed to open camera session\n");
    return 1;
  }

  err = EdsSendCommand(camera, kEdsCameraCommand_PressShutterButton,
                       kEdsCameraCommand_ShutterButton_Completely);
  if (err != EDS_ERR_OK) {
    EdsSendCommand(camera, kEdsCameraCommand_PressShutterButton,
                   kEdsCameraCommand_ShutterButton_OFF);
    printf("Failed to set shutter button\n");
    return 1;
  }

  EdsSendCommand(camera, kEdsCameraCommand_PressShutterButton,
                 kEdsCameraCommand_ShutterButton_OFF);
}
