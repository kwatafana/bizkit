import 'package:flutter/material.dart';
import 'package:bizkit/wid/scaffold.dart';
import 'package:kong/kong.dart';
import 'package:bizkit/wid/menu.dart';
import 'package:omatala/omatala.dart';
import 'package:bizkit/screens/dashboard.dart';

/// Login Screen
class LoginScreen extends StatefulWidget {
  const LoginScreen(
      {Key? key,
      required this.title,
      required this.kong,
      required this.omatala})
      : super(key: key);
  final KongAPI kong;
  final OmatalaAPI omatala;
  final String title;
  static String navAddress = "/login";

  @override
  State<LoginScreen> createState() =>
      _LoginScreenState(kong: kong, omatala: omatala);
}

class _LoginScreenState extends State<LoginScreen> {
  _LoginScreenState({required this.kong, required this.omatala});
  final KongAPI kong;
  final OmatalaAPI omatala;
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    Row title = Row(children: <Widget>[
      Container(
        margin: const EdgeInsets.only(right: 5),
        child: const Icon(Icons.cookie),
      ),
      const Text("BizKit | create account")
    ]);
    return bizkitScaffold(
        title, body(context, kong, omatala), bizkitMenu(context, ""));
  }
}

Widget body(context, kong, omatala) {
  TextEditingController usernameController = TextEditingController();
  TextEditingController passwordController = TextEditingController();

  return ListView(children: <Widget>[
    Container(
      margin: const EdgeInsets.only(right: 5),
      child: const Icon(Icons.lock_open, color: Colors.white),
    ),
    const Text(
      "Account Login",
      style: TextStyle(
        color: Colors.white,
        fontSize: 20,
      ),
    ),
    Container(
      padding: const EdgeInsets.all(5),
      child: const Divider(color: Colors.tealAccent),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: usernameController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Username",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: passwordController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Password",
          labelStyle: TextStyle(color: Colors.grey),
          enabledBorder: OutlineInputBorder(
            borderSide: BorderSide(color: Colors.teal),
          ),
        ),
        obscureText: true,
        style: const TextStyle(color: Colors.white),
      ),
    ),
    Container(
        height: 50,
        padding: const EdgeInsets.fromLTRB(10, 0, 10, 0),
        margin: const EdgeInsets.only(top: 10.0),
        child: ElevatedButton(
          child: const Text("login"),
          onPressed: () async {
            final username = usernameController.text;
            final password = passwordController.text;

            login(username, password, kong, omatala, context);
          },
        )),
  ]);
}

// Send login request and if login succeeds fetch node data
login(String username, String password, KongAPI kong, OmatalaAPI omatala,
    context) async {
  if (username.isNotEmpty && password.isNotEmpty) {
    try {
      final accountInput = AccountLoginInput.validCreate(username, password);
      await kong.login(accountInput);

      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text("Successfully logged in")),
      );

      // Request and set node industry
      await omatala.fetchIndustry();

      // Navigate to dashboard screen
      Navigator.pushNamed(context, DashboardScreen.navAddress);
    } catch (e) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text(e.toString())),
      );
    }
  } else {
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Fill in all fields')),
    );
  }
}
