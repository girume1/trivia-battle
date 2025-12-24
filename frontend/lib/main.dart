import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';
import 'package:provider/provider.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initHiveForFlutter(); 
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final httpLink = HttpLink('http://localhost:9002/graphql');

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
  final String roomId = "room1";
  String status = "Welcome! Join the room to play.";

  // Mutations
  final String joinMutation = r'''mutation Join($roomId: String!) { executeOperation(operation: { joinRoom: { roomId: $roomId } }) }''';
  final String startWithBetMutation = r'''mutation StartWithBet($bet: Int!) { executeOperation(operation: { startRoundWithBet: { betAmount: $bet } }) }''';
  final String submitMutation = r'''mutation Submit($index: Int!) { executeOperation(operation: { submitAnswer: { answerIndex: $index } }) }''';
  final String endRoundMutation = r'''mutation EndRound { executeOperation(operation: { endRound: null }) }''';
  final String claimBonusMutation = r'''mutation ClaimBonus { executeOperation(operation: { creditDailyBonus: null }) }''';

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
      appBar: AppBar(title: Text('Trivia Battle ⚡️'), backgroundColor: Colors.blue[900]),
      body: SingleChildScrollView( // Added scroll for mobile safety
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Daily Bonus Button
            ElevatedButton.icon(
              icon: Icon(Icons.card_giftcard),
              label: Text('Claim Daily Bonus (100 tokens)'),
              style: ElevatedButton.styleFrom(backgroundColor: Colors.purple[700]),
              onPressed: () async {
                final result = await client.mutate(MutationOptions(document: gql(claimBonusMutation)));
                setState(() {
                  status = result.hasException ? "Cooldown active! Try later." : "100 tokens added!";
                });
              },
            ),
            SizedBox(height: 10),

            Card(
              color: Colors.blueGrey[800],
              child: Padding(
                padding: EdgeInsets.all(16),
                child: Text(status, style: TextStyle(fontSize: 18, color: Colors.white)),
              ),
            ),
            SizedBox(height: 10),

            ElevatedButton.icon(
              icon: Icon(Icons.login),
              label: Text('Join Room "$roomId"'),
              style: ElevatedButton.styleFrom(backgroundColor: Colors.green),
              onPressed: () async {
                final result = await client.mutate(MutationOptions(
                  document: gql(joinMutation),
                  variables: {'roomId': roomId},
                ));
                setState(() { status = result.hasException ? "Error joining" : "Joined room!"; });
              },
            ),
            
            // Live State with FIXED BUILDER
            Query(
              options: QueryOptions(
                document: gql(stateQuery),
                pollInterval: Duration(seconds: 2),
              ),
              builder: (QueryResult result, { VoidCallback? refetch, FetchMore? fetchMore }) { // FIXED SIGNATURE
                if (result.hasException) return Text('Error: ${result.exception}');
                if (result.isLoading && result.data == null) return Center(child: CircularProgressIndicator());

                final data = result.data!;
                final state = data['state'];
                final question = state['currentQuestion'] as String? ?? '';
                final roundActive = state['roundActive'] as bool;
                final players = state['playersCount'] as int;
                final answers = state['answersCount'] as int;
                final balance = data['playerBalance'] as int;

                return Column(
                  children: [
                    Text('Balance: $balance tokens | Players: $players', style: TextStyle(fontSize: 18)),
                    if (question.isNotEmpty) ...[
                      SizedBox(height: 20),
                      Card(
                        color: Colors.blue[900],
                        child: Padding(padding: EdgeInsets.all(20), child: Text(question, style: TextStyle(fontSize: 22))),
                      ),
                      ...['Berlin', 'Paris', 'Madrid', 'London'].asMap().entries.map((e) => ElevatedButton(
                        child: Text(e.value),
                        onPressed: () => client.mutate(MutationOptions(document: gql(submitMutation), variables: {'index': e.key})),
                      )),
                    ],
                    if (!roundActive) ElevatedButton(
                      child: Text('Start Round (100 tokens)'),
                      onPressed: () => client.mutate(MutationOptions(document: gql(startWithBetMutation), variables: {'bet': 100})),
                    ),
                    if (answers >= players && roundActive) ElevatedButton(
                      child: Text('End Round'),
                      onPressed: () => client.mutate(MutationOptions(document: gql(endRoundMutation))),
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