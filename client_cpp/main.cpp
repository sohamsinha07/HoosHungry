#include <iostream>
#include <string>
#include <cstdlib>

#include <cpr/cpr.h>
#include <nlohmann/json.hpp>

using json = nlohmann::json;

static std::string getenv_or(const char* key, const std::string& def) {
    const char* v = std::getenv(key);
    return v ? std::string(v) : def;
}

int main(int argc, char** argv) {
    std::string endpoint = getenv_or("HOOSHUNGRY_GQL", "http://localhost:8080/graphql");
    long hall_id = (argc >= 2) ? std::stol(argv[1]) : 1;

    // GraphQL query: recommend
    std::string query = R"(
      query Recommend($hallId: Int!, $prefs: PreferenceInput!, $limit: Int) {
        recommend(hallId: $hallId, prefs: $prefs, limit: $limit) {
          id
          name
          calories
          vegan
          vegetarian
          popularityScore
          score
        }
      }
    )";

    json variables;
    variables["hallId"] = hall_id;
    variables["limit"] = 10;
    variables["prefs"] = {
        {"veganOnly", false},
        {"vegetarianOnly", false},
        {"maxCalories", 700},
        {"query", "pizza"},
        {"popularityWeight", 0.45},
        {"dietaryWeight", 0.35},
        {"calorieWeight", 0.20}
    };

    json body;
    body["query"] = query;
    body["variables"] = variables;

    auto resp = cpr::Post(
        cpr::Url{endpoint},
        cpr::Header{{"Content-Type", "application/json"}},
        cpr::Body{body.dump()}
    );

    if (resp.status_code != 200) {
        std::cerr << "HTTP " << resp.status_code << "\n" << resp.text << "\n";
        return 1;
    }

    json out = json::parse(resp.text);
    if (out.contains("errors")) {
        std::cerr << "GraphQL errors:\n" << out["errors"].dump(2) << "\n";
        return 1;
    }

    auto items = out["data"]["recommend"];
    std::cout << "Top recommendations for hall_id=" << hall_id << ":\n";
    for (auto& it : items) {
        std::cout
            << "- " << it["name"].get<std::string>()
            << " | score=" << it["score"].get<double>()
            << " | kcal=" << (it["calories"].is_null() ? -1 : it["calories"].get<int>())
            << " | vegan=" << (it["vegan"].is_null() ? false : it["vegan"].get<bool>())
            << " | veg=" << (it["vegetarian"].is_null() ? false : it["vegetarian"].get<bool>())
            << "\n";
    }

    return 0;
}
