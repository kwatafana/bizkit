import 'package:flutter/material.dart';
import 'package:bizkit/screens/home.dart';
import 'package:bizkit/screens/create_account.dart';
import 'package:bizkit/screens/login.dart';
import 'package:bizkit/screens/dashboard.dart';
import 'package:kong/kong.dart';
import 'package:omatala/omatala.dart';

void main() {
  runApp(const Bizkit());
}

class Bizkit extends StatelessWidget {
  const Bizkit({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final omatalaNodeAddress = "http://localhost:7878/";
    final kongKonfig = KongAPIKonfig(omatalaNodeAddress, null, null);
    final omatalaKonfig = OmatalaAPIConfig(omatalaNodeAddress);
    final kong = KongAPI(kongKonfig);
    final omatala = OmatalaAPI(omatalaKonfig);

    return MaterialApp(
      title: 'BizKit',
      theme: ThemeData(
        primarySwatch: Colors.teal,
      ),
      initialRoute: '/',
      routes: {
        '/': (context) => const HomeScreen(title: 'BizKit'),
        '/create-account': (context) =>
            CreateAccountScreen(title: 'BizKit', kong: kong),
        '/login': (context) => LoginScreen(title: 'BizKit', kong: kong),
        '/dashboard': (context) => DashboardScreen(
              title: 'BizKit',
              kong: kong,
              omatala: omatala,
            ),
      },
    );
  }
}
