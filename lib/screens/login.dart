import 'package:flutter/material.dart';
import 'package:bizkit/wid/scaffold.dart';
import 'package:kong/kong.dart';

/// Login Screen
class LoginScreen extends StatefulWidget {
  const LoginScreen({Key? key, required this.title, required this.kong})
      : super(key: key);
  final KongAPI kong;
  final String title;

  @override
  State<LoginScreen> createState() => _LoginScreenState(kong: kong);
}

class _LoginScreenState extends State<LoginScreen> {
  _LoginScreenState({required this.kong});
  final KongAPI kong;
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    TextEditingController usernameController = TextEditingController();
    TextEditingController passwordController = TextEditingController();

    return bizkitScaffold(
        Row(children: <Widget>[
          Container(
            margin: const EdgeInsets.only(right: 5),
            child: const Icon(Icons.cookie),
          ),
          const Text("BizKit | create account")
        ]),
        ListView(children: <Widget>[
          Container(
              margin: const EdgeInsets.only(top: 30),
              child:
                  Row(mainAxisAlignment: MainAxisAlignment.center, children: [
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
              ])),
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

                  if (username.isNotEmpty && password.isNotEmpty) {
                    try {
                      final account_input =
                          AccountLoginInput.validCreate(username, password);
                      await kong.login(account_input);

                      ScaffoldMessenger.of(context).showSnackBar(
                        const SnackBar(content: Text("Successfully logged in")),
                      );

                      // Navigate to dashboard screen
                      Navigator.pushNamed(context, '/dashboard');
                    } catch (e) {
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(content: Text(e.toString())),
                      );
                    }
                  } else
                    ScaffoldMessenger.of(context).showSnackBar(
                      const SnackBar(content: Text('Fill in all fields')),
                    );
                  return;
                },
              )),
        ]));
  }
}
