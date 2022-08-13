package restaurantclient.restaurantclient.client;

import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.api.EnvType;
import net.fabricmc.api.Environment;
import net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback;
import restaurantclient.restaurantclient.GenerateRecipeCommand;

@Environment(EnvType.CLIENT)
public class RestaurantClientClient implements ClientModInitializer {

    @Override
    public void onInitializeClient() {
        CommandRegistrationCallback.EVENT.register(new GenerateRecipeCommand());
    }
}
