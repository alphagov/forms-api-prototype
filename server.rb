require "sinatra"
require "json"
require "pry"

before do
  content_type :json
end

post "/publish" do
  user = request.env['HTTP_X_API_KEY']
  request_body = request.body.read
  request = JSON.parse(request_body)

  id = request['id']
  config = {}
  config = request['configuration'] if request['configuration']
  filename = id
  filename = "#{id}_#{user}" if user

  File.write("./forms/#{filename}", JSON.dump(config))

  config.to_json
end

get "/published" do
  user = request.env['HTTP_X_API_KEY']

  files = []
  forms = []

  if user
    forms = Dir.entries("./forms").select { |filename| File.file?("./forms/#{filename}") && filename.include?(user) }
  else
    forms = Dir.entries("./forms").select { |filename| File.file?("./forms/#{filename}") }
  end
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
  user = request.env['HTTP_X_API_KEY']
  form_content = {}
  filename = params['id']
  filename = "#{params['id']}_#{user}" if user

  File.open("./forms/#{filename}") do |f|
    file_content = f.read
    form_content = JSON.parse(file_content)
  end

  {
    id: filename,
    values: form_content
  }.to_json
end
