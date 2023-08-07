import 'package:flutter/material.dart';
import 'package:bizkit/wid/scaffold.dart';
import 'package:kong/kong.dart';

/// Dashboard Screen
class DashboardScreen extends StatefulWidget {
  const DashboardScreen({Key? key, required this.title, required this.kong})
      : super(key: key);
  final KongAPI kong;
  final String title;

  @override
  State<DashboardScreen> createState() => _DashboardScreenState(kong: kong);
}

class _DashboardScreenState extends State<DashboardScreen> {
  _DashboardScreenState({required this.kong});
  final KongAPI kong;
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    TextEditingController usernameController = TextEditingController();
    TextEditingController emailController = TextEditingController();
    TextEditingController passwordController = TextEditingController();
    TextEditingController retypedPasswordController = TextEditingController();

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
                  "Account Dashboard",
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
                  final email = emailController.text;
                  final password = passwordController.text;
                  final retypedPassword = retypedPasswordController.text;

                  if (username.isNotEmpty &&
                      email.isNotEmpty &&
                      password.isNotEmpty) {
                    if (retypedPassword == password) {
                      try {
                        final account_input = AccountCreationInput.validCreate(
                            username, email, password);
                        final public_account_data =
                            await kong.create(account_input);

                        ScaffoldMessenger.of(context).showSnackBar(
                          SnackBar(
                              content: Text(
                                  "Account for ${public_account_data.username} created")),
                        );
                      } catch (e) {
                        ScaffoldMessenger.of(context).showSnackBar(
                          SnackBar(content: Text(e.toString())),
                        );
                      }
                    } else {
                      ScaffoldMessenger.of(context).showSnackBar(
                        const SnackBar(content: Text('Password mismatch')),
                      );
                      return;
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
