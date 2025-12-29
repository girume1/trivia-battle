import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:fluttertoast/fluttertoast.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initHiveForFlutter();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    // UPDATE THIS with your real Linera GraphQL endpoint
    final HttpLink httpLink = HttpLink('http://localhost:8080/graphql');

    final GraphQLClient client = GraphQLClient(
      link: httpLink,
      cache: GraphQLCache(store: HiveStore()),
    );

    return GraphQLProvider(
      client: ValueNotifier(client),
      child: MaterialApp(
        title: 'Trivia Battle ⚡',
        debugShowCheckedModeBanner: false,
        theme: ThemeData.dark().copyWith(
          primaryColor: Colors.blue[800],
          scaffoldBackgroundColor: Colors.black,
          appBarTheme: const AppBarTheme(
            backgroundColor: Colors.blueAccent,
            foregroundColor: Colors.white,
          ),
          elevatedButtonTheme: ElevatedButtonThemeData(
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.blueAccent,
              padding: const EdgeInsets.symmetric(vertical: 16, horizontal: 32),
              textStyle: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
            ),
          ),
        ),
        home: const LobbyScreen(),
      ),
    );
  }
}

class LobbyScreen extends StatelessWidget {
  const LobbyScreen({super.key});

  final String roomsQuery = r'''
    query GetRooms {
      all_rooms {
        id
        name
        current_players
        max_players
        bet_amount
        has_password
      }
    }
  ''';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Trivia Battle ⚡'),
        centerTitle: true,
      ),
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topCenter,
            end: Alignment.bottomCenter,
            colors: [Colors.black, Colors.blueGrey],
          ),
        ),
        child: Query(
          options: QueryOptions(document: gql(roomsQuery)),
          builder: (result, {fetchMore, refetch}) {
            if (result.isLoading) {
              return const Center(child: CircularProgressIndicator(color: Colors.blueAccent));
            }

            if (result.hasException) {
              return Center(
                child: Text(
                  'Error: ${result.exception.toString()}',
                  style: const TextStyle(color: Colors.red),
                  textAlign: TextAlign.center,
                ),
              );
            }

            final rooms = result.data?['all_rooms'] as List<dynamic>? ?? [];

            return ListView.builder(
              padding: const EdgeInsets.all(16),
              itemCount: rooms.length + 1,
              itemBuilder: (context, index) {
                if (index == 0) {
                  return Column(
                    children: [
                      const Text(
                        'Open Rooms',
                        style: TextStyle(fontSize: 32, fontWeight: FontWeight.bold, color: Colors.white),
                      ),
                      const SizedBox(height: 20),
                      ElevatedButton(
                        onPressed: () {
                          Fluttertoast.showToast(msg: "Create Room coming soon!");
                        },
                        child: const Text('CREATE NEW ROOM'),
                      ),
                      const SizedBox(height: 30),
                    ],
                  );
                }

                final room = rooms[index - 1];
                final hasPassword = room['has_password'] as bool? ?? false;
                final lockText = hasPassword ? ' (Locked)' : '';

                return Card(
                  color: Colors.blue[900],
                  margin: const EdgeInsets.symmetric(vertical: 8),
                  child: ListTile(
                    title: Text(
                      room['name'] as String? ?? 'Unnamed Room',
                      style: const TextStyle(fontWeight: FontWeight.bold, color: Colors.white),
                    ),
                    subtitle: Text(
                      '${room['current_players'] ?? 0}/${room['max_players'] ?? 4} players • Bet: ${room['bet_amount'] ?? 0} tokens$lockText',
                      style: const TextStyle(color: Colors.grey),
                    ),
                    trailing: const Icon(Icons.arrow_forward_ios, color: Colors.white),
                    onTap: () {
                      Fluttertoast.showToast(msg: "Joining room - Coming soon!");
                    },
                  ),
                );
              },
            );
          },
        ),
      ),
    );
  }
}