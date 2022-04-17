///////////////filename: blecontroller.dart
import 'package:flutter/material.dart';
import 'package:flutter_reactive_ble/flutter_reactive_ble.dart';
import 'package:get/get.dart';
import 'dart:async';
import 'dart:typed_data';

class BleController {
  final frb = FlutterReactiveBle();
  late StreamSubscription<ConnectionStateUpdate> c;
  late QualifiedCharacteristic tx;
  final devId = 'A4:DA:32:55:06:1E'; // use nrf connect from playstore to find
  var status = 'connect to bluetooth'.obs;
  var address = ' 30'.obs;
  var sliderVal = 0.0.obs;
  int currentAddress = 0;
  var currentSpeed = 0;
  int currentDirection = 0;
  int dataPacket = 0;
  List<int> packet = [0x24, 0, 0, 0]; // 0x24 packet leader
  
  void setAddress(val) async {
    address.value = val.toString();
    currentAddress = val.toInt();
    packet[1] = currentAddress;
    //print("address");
    //print(packet[0].toRadixString(16));    
    //print(packet[1].toRadixString(16));
    //print(packet[2].toRadixString(16));
    //print(packet[3].toRadixString(16));
    await frb.writeCharacteristicWithoutResponse(tx, value: packet);
  }

  Int8List int32Bytes(int value) =>
      Int8List(4)..buffer.asInt32List()[0] = value;

  void setDirection(val) async {
    currentDirection = val.toInt();
    packet[2] = currentDirection;
    sliderVal.value = 0;
    //print("direction");
    //print(packet[0].toRadixString(16));    
    //print(packet[1].toRadixString(16));
    //print(packet[2].toRadixString(16));
    //print(packet[3].toRadixString(16));
    await frb.writeCharacteristicWithoutResponse(tx, value: packet);
  }

  void setSpeed(val) async {
    sliderVal.value = val;
    int s = val.toInt();
    dataPacket = currentDirection + s;
    packet[2] = dataPacket;
    //print("data");
    //print(dataPacket.toRadixString(16));
    // int i = val.toInt();
    // print('value: $i'); // debugging
    // Int8List li = int32Bytes(i);
    //print("speed");
    //print(packet[0].toRadixString(16));    
    //print(packet[1].toRadixString(16));
    //print(packet[2].toRadixString(16));
    //print(packet[3].toRadixString(16));
    await frb.writeCharacteristicWithoutResponse(tx, value: packet);
  }

  void connect() async {
    status.value = 'connecting...';
    c = frb.connectToDevice(id: devId).listen((state) {
      if (state.connectionState == DeviceConnectionState.connected) {
        status.value = 'connected!';

        tx = QualifiedCharacteristic(
            serviceId: Uuid.parse("FFE0"),
            characteristicId: Uuid.parse("FFE1"),
            deviceId: devId);
      }
    });
  }
}
