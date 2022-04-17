//////////////// filename: main.dart
// demo of flutter dcc using bluetooth
// run on pixel 4, android version 12
// flutter version - 2.13.0-0.1.pre - channel dev
// dart - 2.17.0, devtools 2.12.1
// change android/app/build.gradle to minSdkVersion 21
// 
// flutter install on fresh linux liteos 
//    -- https://www.linuxliteos.com/
//
/*
flutter ble template and flutter linux install instruction

flutter install on linux howto: https://ubuntu.com/blog/getting-started-with-flutter-on-ubuntu

sudo snap install flutter --classic

sudo snap alias flutter.dart dart

sudo snap install android-studio --classic

launch android studio:

click more actions (lower part of the screen)

click on SDK Manager

under SDK Platfroms tab add any Android you want

under SDK Tools tab

    click on:
    
        Android SDK Command-line Tools
        
        Android Emulator
        
        Android SDK Platforms-Tools
        
        click Apply

flutter config --android-studio-dir /snap/android-studio/current/android-studio

flutter doctor --android-licenses

flutter channel dev

flutter upgrade

flutter config --enable-linux-desktop

flutter doctor

mkdir flutter_projects

cd flutter_projects

flutter create myapp

cd myapp

flutter run -d linux (this will run on your desktop)
*/

import 'package:flutter/material.dart';
import 'package:get/get.dart';
import './blecontroller.dart';

void main() => runApp(GetMaterialApp(debugShowCheckedModeBanner:false,home: Home()));

class Home extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final BleController ble = Get.put(BleController());
    return Scaffold(
        appBar: AppBar(
           centerTitle:true,
           title: const Text('DCC demo'),
        
           actions:[
              PopupMenuButton(
                color: Colors.yellowAccent,
                child:Text("Address change"),
                
                itemBuilder:(context)=> [
                    PopupMenuItem(child: Text("3"),  value:3,),
                    PopupMenuItem(child: Text("12"), value:12,), 
                    PopupMenuItem(child: Text("20"), value:20,),  
                    PopupMenuItem(child: Text("32"), value:32,),
                    PopupMenuItem(child: Text("50"), value:50,),
                    PopupMenuItem(child: Text("51"), value:51,),
                    PopupMenuItem(child: Text("71"), value:71,),],
                  
                onSelected:(val){ble.setAddress(val);}
              )],), // appBar closing paren
        
        body: Column(children: [
          //------ column child
          SizedBox(height: 50.0),
          //------ column child
          Center(
              child: ElevatedButton(
                  onPressed: ble.connect,
                  child: Obx(() => Text('${ble.status.value}',
                      style: TextStyle(
                          fontSize: 25, fontWeight: FontWeight.bold))))),
          //------- column child
          SizedBox(height: 50.0),
          //------- column child
          Center(child: Obx(()=> Row(children:[
            SizedBox(width:100.0),
            Text('address: '),
            Text('${ble.address.value}'),
          ]))),
          //------- column child
          SizedBox(height:50.0),
          //-------- column child
          ButtonBar(alignment: MainAxisAlignment.spaceEvenly, children: [
            ElevatedButton(
                child: Text('forward'),
                onPressed: () {
                  ble.setDirection(0x50);
                }),
            ElevatedButton(child: Text('stop'), onPressed: () {ble.setDirection(0x50);}),
            ElevatedButton(
                child: Text('reverse'),
                onPressed: () {
                  ble.setDirection(0x70);
                }),
          ]),
          //------ column child
          SizedBox(height: 50.0),
          //------- column child
          RotatedBox(
              quarterTurns: 3,
              child: Obx(() => Slider(
                  min: 0,
                  max: 15,
                  label: ble.sliderVal.toString(),
                  value: ble.sliderVal.value,
                  onChanged: (double value) {
                    ble.setSpeed(value);
                  }))),
          //
        ]));
  }
}
