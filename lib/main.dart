import 'package:flutter/material.dart';
import 'package:bizkit/screens/home.dart';
import 'package:bizkit/screens/create_account.dart';
import 'package:bizkit/screens/login.dart';
import 'package:bizkit/screens/dashboard.dart';
import 'package:kong/kong.dart';
import 'package:omatala/omatala.dart';
import 'package:bizkit/sme/real_estate/add_property.dart';

void main() {
  runApp(const Bizkit());
}

class Bizkit extends StatelessWidget {
  const Bizkit({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    const omatalaNodeAddress = "http://localhost:7878/";
    final kongKonfig = KongAPIKonfig(omatalaNodeAddress, null, null);
    final omatalaKonfig = OmatalaAPIConfig(omatalaNodeAddress);
    final kong = KongAPI(kongKonfig);
    final omatala = OmatalaAPI(omatalaKonfig);

    // Enable kong APIs used by bizkit
    kong.enablePropertiesAPI();

    return MaterialApp(
      title: 'BizKit',
      theme: ThemeData(
        primarySwatch: Colors.teal,
      ),
      initialRoute: HomeScreen.navAddress,
      routes: {
        HomeScreen.navAddress: (context) => const HomeScreen(title: 'BizKit'),
        CreateAccountScreen.navAddress: (context) =>
            CreateAccountScreen(title: 'BizKit', kong: kong),
        LoginScreen.navAddress: (context) =>
            LoginScreen(title: 'BizKit', kong: kong, omatala: omatala),
        DashboardScreen.navAddress: (context) => DashboardScreen(
              title: 'BizKit',
              kong: kong,
              omatala: omatala,
            ),
        AddPropertyScreen.navAddress: (context) => AddPropertyScreen(
              title: 'BizKit',
              kong: kong,
              omatala: omatala,
            ),
      },
    );
  }
}
