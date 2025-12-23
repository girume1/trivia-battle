import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:provider/provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initHiveForFlutter(); // For GraphQL cache
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final httpLink = HttpLink('http://localhost:9002/graphql'); // Local backend

    final ValueNotifier<GraphQLClient> client = ValueNotifier(
      GraphQLClient(
        link: httpLink,
        cache: GraphQLCache(store: HiveStore()),
      ),
    );

    return Provider<ValueNotifier<GraphQLClient>>.value(
      value: client,
      child: MaterialApp(
        title: 'Trivia Battle ⚡️',
        theme: ThemeData.dark().copyWith(
          primaryColor: Colors.blue[800],
          scaffoldBackgroundColor: Colors.grey[900],
          elevatedButtonTheme: ElevatedButtonThemeData(
            style: ElevatedButton.styleFrom(
              padding: EdgeInsets.symmetric(vertical: 16, horizontal: 24),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(12)),
            ),
          ),
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
  String status = "Welcome! Join the room to play.";

  // Mutations
  final String joinMutation = r'''
    mutation Join($roomId: String!) {
      executeOperation(operation: { joinRoom: { roomId: $roomId } })
    }
  ''';

  final String startWithBetMutation = r'''
    mutation StartWithBet($bet: Int!) {
      executeOperation(operation: { startRoundWithBet: { betAmount: $bet } })
    }
  ''';

  final String submitMutation = r'''
    mutation Submit($index: Int!) {
      executeOperation(operation: { submitAnswer: { answerIndex: $index } })
    }
  ''';

  final String endRoundMutation = r'''
    mutation EndRound {
      executeOperation(operation: { endRound: null })
    }
  ''';

  // Queries
  final String stateQuery = r'''
    query GetState {
      state {
        roomId
        playersCount
        roundActive
        currentQuestion
        answersCount
        pot
      }
      playerBalance
      playerScore
    }
  ''';

  @override
  Widget build(BuildContext context) {
    final client = Provider.of<ValueNotifier<GraphQLClient>>(context).value;

    return Scaffold(
      appBar: AppBar(
        title: Text('Trivia Battle ⚡️'),
        backgroundColor: Colors.blue[900],
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Status Message
            Card(
              color: Colors.blueGrey[800],
              child: Padding(
                padding: EdgeInsets.all(16),
                child: Text(status, style: TextStyle(fontSize: 18, color: Colors.white)),
              ),
            ),
            SizedBox(height: 20),

            // Join Room Button
            ElevatedButton.icon(
              icon: Icon(Icons.login),
              label: Text('Join Room "$roomId"'),
              style: ElevatedButton.styleFrom(backgroundColor: Colors.green),
              onPressed: () async {
                final mutation = MutationOptions(
                  document: gql(joinMutation),
                  variables: {'roomId': roomId},
                );
                final result = await client.mutate(mutation);
                setState(() {
                  status = result.hasException
                      ? "Error: ${result.exception.toString()}"
                      : "Joined room! Ready to play.";
                });
              },
            ),
            SizedBox(height: 20),

            // Live State with Polling
            Query(
              options: QueryOptions(
                document: gql(stateQuery),
                pollInterval: Duration(seconds: 2),
              ),
              builder: (result, {refetch}) {
                if (result.hasException) {
                  return Text('Error: ${result.exception}', style: TextStyle(color: Colors.red));
                }
                if (result.isLoading && result.data == null) {
                  return Center(child: CircularProgressIndicator());
                }

                final data = result.data;
                if (data == null) return Text('No data yet');

                final state = data['state'];
                final question = state['currentQuestion'] as String? ?? '';
                final roundActive = state['roundActive'] as bool;
                final players = state['playersCount'] as int;
                final answers = state['answersCount'] as int;
                final pot = state['pot'] as int;
                final balance = data['playerBalance'] as int;
                final score = data['playerScore'] as int;

                return Column(
                  crossAxisAlignment: CrossAxisAlignment.stretch,
                  children: [
                    // Player Info
                    Card(
                      color: Colors.blueGrey[700],
                      child: Padding(
                        padding: EdgeInsets.all(16),
                        child: Column(
                          children: [
                            Text('Players: $players', style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold)),
                            Text('Your Balance: $balance tokens', style: TextStyle(fontSize: 16)),
                            Text('Your Score: $score', style: TextStyle(fontSize: 16)),
                            Text('Current Pot: $pot tokens', style: TextStyle(fontSize: 16)),
                            Text('Answers: $answers / $players'),
                          ],
                        ),
                      ),
                    ),
                    SizedBox(height: 20),

                    // Question
                    if (question.isNotEmpty)
                      Card(
                        elevation: 4,
                        color: Colors.blue[900],
                        child: Padding(
                          padding: EdgeInsets.all(20),
                          child: Text(
                            question,
                            style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
                            textAlign: TextAlign.center,
                          ),
                        ),
                      ),
                    SizedBox(height: 30),

                    // Answer Buttons
                    if (question.isNotEmpty && roundActive)
                      ...['Berlin', 'Paris', 'Madrid', 'London'].asMap().entries.map((entry) {
                        int idx = entry.key;
                        String opt = entry.value;
                        return Padding(
                          padding: EdgeInsets.symmetric(vertical: 8),
                          child: ElevatedButton(
                            style: ElevatedButton.styleFrom(backgroundColor: Colors.blue[700]),
                            onPressed: () async {
                              final mutation = MutationOptions(
                                document: gql(submitMutation),
                                variables: {'index': idx},
                              );
                              final result = await client.mutate(mutation);
                              setState(() {
                                status = result.hasException
                                    ? "Error: ${result.exception}"
                                    : "Submitted: $opt";
                              });
                            },
                            child: Text(opt, style: TextStyle(fontSize: 20)),
                          ),
                        );
                      }).toList(),

                    // Status Messages
                    if (question.isEmpty && roundActive)
                      Center(child: Text('Round active — waiting for question...', style: TextStyle(color: Colors.yellow))),
                    if (question.isEmpty && !roundActive)
                      Center(child: Text('Start the round to play!', style: TextStyle(color: Colors.orange))),

                    SizedBox(height: 20),

                    // Start Round with Bet
                    if (!roundActive)
                      ElevatedButton.icon(
                        icon: Icon(Icons.play_arrow),
                        label: Text('Start Round (Bet 100 tokens)'),
                        style: ElevatedButton.styleFrom(backgroundColor: Colors.orange),
                        onPressed: () async {
                          final mutation = MutationOptions(
                            document: gql(startWithBetMutation),
                            variables: {'bet': 100},
                          );
                          final result = await client.mutate(mutation);
                          setState(() {
                            status = result.hasException
                                ? "Error: ${result.exception}"
                                : "Bet placed! Round started!";
                          });
                        },
                      ),

                    SizedBox(height: 20),

                    // End Round (when all answered)
                    if (answers >= players && roundActive)
                      ElevatedButton.icon(
                        icon: Icon(Icons.check_circle),
                        label: Text('End Round & Resolve'),
                        style: ElevatedButton.styleFrom(backgroundColor: Colors.green),
                        onPressed: () async {
                          final mutation = MutationOptions(document: gql(endRoundMutation));
                          final result = await client.mutate(mutation);
                          setState(() {
                            status = result.hasException
                                ? "Error ending round"
                                : "Round ended! Winners paid.";
                          });
                        },
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
