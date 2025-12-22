import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:provider/provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initHiveForFlutter(); // For cache
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final httpLink = HttpLink('http://localhost:9002/graphql'); // Your local backend

    final ValueNotifier<GraphQLClient> client = ValueNotifier(
      GraphQLClient(
        link: httpLink,
        cache: GraphQLCache(store: HiveStore()),
      ),
    );

    return Provider<ValueNotifier<GraphQLClient>>.value(
      value: client,
      child: MaterialApp(
        title: 'Trivia Battle',
        theme: ThemeData(
          primarySwatch: Colors.blue,
          scaffoldBackgroundColor: Colors.grey[900],
          textTheme: TextTheme(bodyMedium: TextStyle(color: Colors.white)),
        ),
        home: HomeScreen(),
      ),
    );
  }
}

class HomeScreen extends StatefulWidget {
  @override
  _HomeScreenState createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  final String roomId = "room1"; // Hardcoded for MVP
  String status = "Welcome! Join the room to start.";

  // Mutations
  final String joinMutation = r'''
    mutation Join($roomId: String!) {
      executeOperation(operation: { joinRoom: { roomId: $roomId } })
    }
  ''';

  final String startMutation = r'''
    mutation Start {
      executeOperation(operation: { startRound: null })
    }
  ''';

  final String submitMutation = r'''
    mutation Submit($index: Int!) {
      executeOperation(operation: { submitAnswer: { answerIndex: $index } })
    }
  ''';

  // Query
  final String stateQuery = r'''
    query GetState {
      state {
        roomId
        playersCount
        roundActive
        currentQuestion
        answersCount
      }
    }
  ''';

  @override
  Widget build(BuildContext context) {
    final client = Provider.of<ValueNotifier<GraphQLClient>>(context).value;

    return Scaffold(
      appBar: AppBar(title: Text('Trivia Battle ⚡️'), backgroundColor: Colors.blue[900]),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            Text(status, style: TextStyle(fontSize: 18, color: Colors.white70)),
            SizedBox(height: 20),

            // Join Room Button
            ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: Colors.green,
                padding: EdgeInsets.symmetric(vertical: 16),
              ),
              onPressed: () async {
                final mutation = MutationOptions(
                  document: gql(joinMutation),
                  variables: {'roomId': roomId},
                );
                final result = await client.mutate(mutation);
                setState(() {
                  if (result.hasException) {
                    status = "Error: ${result.exception}";
                  } else {
                    status = "Joined room $roomId! Ready to play.";
                  }
                });
              },
              child: Text('Join Room "$roomId"', style: TextStyle(fontSize: 18)),
            ),

            SizedBox(height: 20),

            // Real-time State (polls every 2s)
            Query(
              options: QueryOptions(
                document: gql(stateQuery),
                pollInterval: Duration(seconds: 2),
              ),
              builder: (result, {refetch, fetchMore}) {
                if (result.hasException) {
                  return Text('Error: ${result.exception}', style: TextStyle(color: Colors.red));
                }
                if (result.isLoading && result.data == null) {
                  return Center(child: CircularProgressIndicator());
                }

                final data = result.data?['state'];
                if (data == null) return Text('No data yet');

                final questionText = data['currentQuestion'] as String? ?? '';
                final roundActive = data['roundActive'] as bool;
                final players = data['playersCount'] as int;
                final answers = data['answersCount'] as int;

                return Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    Text('Players in room: $players', style: TextStyle(fontSize: 20, color: Colors.white)),
                    SizedBox(height: 10),
                    Text('Answers submitted: $answers', style: TextStyle(fontSize: 16, color: Colors.white70)),
                    SizedBox(height: 20),

                    if (questionText.isNotEmpty) ...[
                      Container(
                        padding: EdgeInsets.all(16),
                        decoration: BoxDecoration(
                          color: Colors.blueGrey[800],
                          borderRadius: BorderRadius.circular(12),
                        ),
                        child: Text(
                          questionText,
                          style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold, color: Colors.white),
                          textAlign: TextAlign.center,
                        ),
                      ),
                      SizedBox(height: 30),

                      // Answer Buttons (hardcoded for MVP question)
                      ...['Berlin', 'Paris', 'Madrid', 'London'].asMap().entries.map((entry) {
                        int idx = entry.key;
                        String option = entry.value;
                        return Padding(
                          padding: EdgeInsets.symmetric(vertical: 8),
                          child: ElevatedButton(
                            style: ElevatedButton.styleFrom(
                              backgroundColor: Colors.blue[700],
                              padding: EdgeInsets.symmetric(vertical: 16),
                              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
                            ),
                            onPressed: roundActive
                                ? () async {
                                    final mutation = MutationOptions(
                                      document: gql(submitMutation),
                                      variables: {'index': idx},
                                    );
                                    final result = await client.mutate(mutation);
                                    setState(() {
                                      if (result.hasException) {
                                        status = "Error: ${result.exception}";
                                      } else {
                                        status = "Answer submitted: $option";
                                      }
                                    });
                                  }
                                : null,
                            child: Text(option, style: TextStyle(fontSize: 20)),
                          ),
                        );
                      }).toList(),
                    ] else if (roundActive) ...[
                      Center(child: Text('Round active — waiting for question...', style: TextStyle(color: Colors.yellow))),
                    ] else ...[
                      Center(child: Text('Start the round to play!', style: TextStyle(color: Colors.orange))),
                    ],

                    SizedBox(height: 20),

                    // Start Round Button
                    if (!roundActive)
                      ElevatedButton(
                        style: ElevatedButton.styleFrom(
                          backgroundColor: Colors.orange,
                          padding: EdgeInsets.symmetric(vertical: 16),
                        ),
                        onPressed: () async {
                          final mutation = MutationOptions(document: gql(startMutation));
                          final result = await client.mutate(mutation);
                          setState(() {
                            if (result.hasException) {
                              status = "Error: ${result.exception}";
                            } else {
                              status = "Round started!";
                            }
                          });
                        },
                        child: Text('Start Round', style: TextStyle(fontSize: 18)),
                      ),
                  ],
                );
              },
            ),
          ],
        ),
      ),
    );
  }
}
