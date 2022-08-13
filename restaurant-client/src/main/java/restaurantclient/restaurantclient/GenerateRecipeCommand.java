package restaurantclient.restaurantclient;

import static net.minecraft.server.command.CommandManager.literal;

import com.mojang.brigadier.CommandDispatcher;
import java.net.URI;
import java.net.URISyntaxException;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.net.http.HttpResponse.BodyHandlers;
import net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback;
import net.minecraft.command.CommandRegistryAccess;
import net.minecraft.server.command.CommandManager.RegistrationEnvironment;
import net.minecraft.server.command.ServerCommandSource;
import net.minecraft.text.Text;

public class GenerateRecipeCommand implements CommandRegistrationCallback {

    private static final String RECIPE_GENERATOR_URL = System.getenv()
        .getOrDefault("RECIPE_GENERATOR_URL", "http://localhost:9911/api/v1/generate-recipe");
    private static final String REQUEST_FORMAT = "{\"username\":\"%s\"}";

    private static HttpRequest buildRequest(String username) throws URISyntaxException {
        return HttpRequest.newBuilder().uri(new URI(RECIPE_GENERATOR_URL))
            .header("Content-Type", "application/json")
            .POST(HttpRequest.BodyPublishers.ofString(REQUEST_FORMAT.formatted(username))).build();
    }

    @Override
    public void register(CommandDispatcher<ServerCommandSource> dispatcher,
        CommandRegistryAccess registryAccess, RegistrationEnvironment environment) {
        dispatcher.register(literal("generate-recipe").executes(context -> {
            var source = context.getSource();
            try {
                var request = buildRequest(source.getName());
                HttpResponse<String> response = HttpClient.newBuilder().build()
                    .send(request, BodyHandlers.ofString());
                source.sendMessage(Text.literal(response.body()));
            } catch (Exception e) {
                source.sendError(Text.literal(e.getLocalizedMessage()));
                return 1;
            }
            return 0;
        }));
    }

}
