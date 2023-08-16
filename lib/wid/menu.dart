import 'package:flutter/material.dart';
import 'package:bizkit/sme/real_estate/add_property.dart';

Drawer bizkitMenu(context, String industry) {
  var menuList = [
    const DrawerHeader(
      decoration: BoxDecoration(
        color: Color(0xFF00BFA5),
      ),
      child: Text('Menu'),
    ),
    ListTile(
        title: const Text('Home'),
        onTap: () {
          Navigator.pop(context);
        },
        textColor: Colors.teal),
    ListTile(
        title: const Text('Settings'),
        onTap: () {
          //Navigator.pushNamed(context, SettingsScreen.routeName);
        },
        textColor: Colors.teal),
  ];

  if (industry == "Real Estate") {
    menuList = realEstateMenu(context, menuList);
  }

  return Drawer(
      backgroundColor: Color(0xFF0222d31),
      child: ListView(
          // Important: Remove any padding from the ListView.
          padding: EdgeInsets.zero,
          children: menuList));
}

List<StatelessWidget> realEstateMenu(context, List<StatelessWidget> menuList) {
  menuList.add(ListTile(
      title: const Text('Add Property'),
      onTap: () {
        Navigator.pushNamed(context, AddPropertyScreen.navAddress);
      },
      textColor: Colors.teal));

  return menuList;
}
