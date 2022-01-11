require "sinatra"
require "json"
require "pry"

before do
  content_type :json
end

post "/publish" do
  request_body = request.body.read
  request = JSON.parse(request_body)

  id = request['id']
  config = {}
  config = request['configuration'] if request['configuration']
  p config

  File.write("./forms/#{id}", JSON.dump(config))

  config.to_json
end

get "/published" do
  files = []
  forms = Dir.entries("./forms").select { |filename| File.file?("./forms/#{filename}") }
  forms.each do |form|
    File.open("./forms/#{form}") do |f|
      files << {
        "Key": form,
        "DisplayName": form,
        "FeedbackForm": false
      }
    end
  end

  files.to_json
end

get "/published/:id" do
  form_content = {}

  File.open("./forms/#{params['id']}") do |f|
    file_content = f.read
    form_content = JSON.parse(file_content)
  end

  {
    id: params['id'],
    values: form_content
  }.to_json
end