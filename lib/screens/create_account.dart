import 'package:flutter/material.dart';
import 'package:bizkit/wid/scaffold.dart';
import 'package:kong/kong.dart';
import 'package:bizkit/wid/menu.dart';

/// Create Account Screen
class CreateAccountScreen extends StatefulWidget {
  const CreateAccountScreen({Key? key, required this.title, required this.kong})
      : super(key: key);
  final KongAPI kong;
  final String title;

  @override
  State<CreateAccountScreen> createState() =>
      _AddProjectScreenState(kong: kong);
}

class _AddProjectScreenState extends State<CreateAccountScreen> {
  _AddProjectScreenState({required this.kong});
  final KongAPI kong;
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

    return bizkitScaffold(title, body(context, kong), bizkitMenu(context, ""));
  }
}

Widget body(context, kong) {
  TextEditingController usernameController = TextEditingController();
  TextEditingController emailController = TextEditingController();
  TextEditingController passwordController = TextEditingController();
  TextEditingController retypedPasswordController = TextEditingController();

  return ListView(children: <Widget>[
    Container(
      margin: const EdgeInsets.only(right: 5),
      child: const Icon(Icons.person, color: Colors.white),
    ),
    const Text(
      "New Account",
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
        controller: emailController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Email",
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
      padding: const EdgeInsets.all(10),
      child: TextField(
        controller: retypedPasswordController,
        decoration: const InputDecoration(
          border: OutlineInputBorder(),
          labelText: "Re-type Password",
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
          child: const Text("create account"),
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
                  final public_account_data = await kong.create(account_input);

                  ScaffoldMessenger.of(context).showSnackBar(
                    SnackBar(
                        content: Text(
                            "Account for ${public_account_data.username} created!")),
                  );
                  Navigator.pushNamed(context, '/login');
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
        ))
  ]);
}
