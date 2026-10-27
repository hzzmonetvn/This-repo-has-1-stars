require 'faraday'

$repo_url = ENV['REPO_URL'] || 'https://api.github.com/repos/hzzmonetvn/This-project-has-N-stars'
$github_token = ENV['GITHUB_TOKEN'] || ''

$conn = Faraday.new do |f|
    f.request :json
    f.response :json
    f.headers['Accept'] = 'application/vnd.github.v3+json'
    f.headers['Authorization'] = 'token ' << $github_token
end

def get_stars
    res = $conn.get($repo_url)
    if res.status.eql? 200
        name = res.body['full_name']
        stars = res.body['watchers']
        puts "Successfully get stars: Current stars: #{stars}, Current repo name: #{name}"
        return name, stars
    else
        raise "Can't get repo info"
    end
end

def update_project
    name, stars = get_stars
    current_stars = name.match(/This-repo-has-(\d+)?-stars/)[1].to_i

    puts "Updating project..."
    if current_stars != stars
        data = {
            name: "This-repo-has-#{stars}-stars",
            description: "This repo has #{stars} stars, star it to test!"
        }
        res = $conn.patch($repo_url, data)
        if res.status.eql? 200
            $repo_url = res.body['url']
            puts "Successfully updated project"
        else
            puts "Update failed"
        end
    end
end

loop do
    update_project
    sleep 5
end
