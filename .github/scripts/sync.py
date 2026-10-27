import json
import os
import re
import sys
import urllib.error
import urllib.request

REPO_ID = os.environ.get("REPO_ID", "1319484859")
TOKEN = os.environ.get("SYNC_TOKEN") or os.environ.get("GITHUB_TOKEN", "")

if not TOKEN:
    print("Error: No GitHub token provided in SYNC_TOKEN or GITHUB_TOKEN")
    sys.exit(1)

api_url = f"https://api.github.com/repositories/{REPO_ID}"
headers = {
    "Accept": "application/vnd.github.v3+json",
    "Authorization": f"Bearer {TOKEN}",
    "User-Agent": "hzzmonetvn-star-sync",
}

# 1. Fetch repo data
req = urllib.request.Request(api_url, headers=headers)
try:
    with urllib.request.urlopen(req) as resp:
        data = json.loads(resp.read().decode("utf-8"))
except urllib.error.HTTPError as e:
    print(f"Error fetching repo data: {e.code} {e.reason}")
    sys.exit(1)

stars = data.get("stargazers_count", 0)
current_name = data.get("name", "")
current_desc = data.get("description", "")
target_name = f"This-repo-has-{stars}-stars"
target_desc = f"This repo has {stars} stars, star it to test!"

print(f"Current repo name: {current_name}")
print(f"Current stargazers count: {stars}")

# 2. Update repo name & description if needed
if current_name != target_name or current_desc != target_desc:
    print(f"Renaming repo from {current_name} to {target_name}...")
    patch_data = json.dumps({
        "name": target_name,
        "description": target_desc,
    }).encode("utf-8")
    patch_req = urllib.request.Request(
        api_url,
        data=patch_data,
        headers={**headers, "Content-Type": "application/json"},
        method="PATCH",
    )
    try:
        with urllib.request.urlopen(patch_req) as patch_resp:
            print("Successfully updated repo name and description!")
    except urllib.error.HTTPError as e:
        print(f"Error updating repo: {e.code} {e.reason} - {e.read().decode('utf-8')}")
        sys.exit(1)
else:
    print("Repo name and description are already up to date.")

# 3. Update README.md
readme_path = "README.md"
if os.path.exists(readme_path):
    with open(readme_path, "r", encoding="utf-8") as f:
        content = f.read()

    new_content = re.sub(r"This-repo-has-\d+-stars", target_name, content)
    new_content = re.sub(
        r"has \*\*\d+\*\* stars", f"has **{stars}** stars", new_content
    )
    new_content = re.sub(r"has \d+ stars", f"has {stars} stars", new_content)

    if new_content != content:
        with open(readme_path, "w", encoding="utf-8") as f:
            f.write(new_content)
        print("Updated README.md with new star count.")
    else:
        print("README.md already up to date.")
