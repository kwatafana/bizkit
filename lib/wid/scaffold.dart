import 'package:flutter/material.dart';

Scaffold bizkitScaffold(title, body, drawer) {
  return Scaffold(
    appBar: AppBar(
      title: title,
      backgroundColor: Colors.teal,
    ),
    backgroundColor: const Color(0xFF0222d31),
    body: body,
    drawer: drawer,
  );
}
