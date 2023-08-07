import 'package:flutter/material.dart';
import 'package:bizkit/screens/home.dart';
import 'package:bizkit/screens/create_account.dart';
import 'package:bizkit/screens/login.dart';
import 'package:bizkit/screens/dashboard.dart';
import 'package:kong/kong.dart';

void main() {
  runApp(const Bizkit());
}

class Bizkit extends StatelessWidget {
  const Bizkit({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final kongKonfig = KongAPIKonfig("http://localhost:7878/", null, null);
    final kong = KongAPI(kongKonfig);

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
        '/dashboard': (context) => DashboardScreen(title: 'BizKit', kong: kong),
      },
    );
  }
}
