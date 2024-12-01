namespace AoC;

internal static class Program {
     
     public static void Main(string[] args) {
          // take arg --generate or -g and a number to generate a day
          if (args.Length == 2 && (args[0] == "--generate" || args[0] == "-g")) {
               if (int.TryParse(args[1], out var day)) {
                    Utils.Generator.GenerateDay(day);
               }
          }
          else {
               // run the day
               var day = args.Length == 1 ? args[0] : null;
               if (day is null) {
                    return;
               }
               var type = Type.GetType($"AoC.Days.Day{day}");
               type?.GetMethod("Run")?.Invoke(null, null);
          }
     }
}