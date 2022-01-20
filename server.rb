require "sinatra"
require "json"
require "pry"
require_relative "./db/database"

class Server < Sinatra::Base
  before do
    content_type :json
    @database = Database.new.connect
  end

  after do
    @database.disconnect
  end

  post "/publish" do
    user = request.env['HTTP_X_API_KEY']

    request_body = request.body.read
    request = JSON.parse(request_body)

    id = request['id']
    config = {}
    config = request['configuration'] if request['configuration']

    if form_exists_for_user?(user, id)
      @database[:forms].where(
        username: user,
        key: id
      ).update(
        form: Sequel.pg_json(config)
      )
    else
      @database[:forms].insert(
        username: user,
        key: id,
        display_name: id,
        form: Sequel.pg_json(config)
      )
    end

    config.to_json
  end

  def form_exists_for_user?(user, key)
    !@database[:forms].where(username: user, key: key).all.empty?
  end

  get "/published" do
    user = request.env['HTTP_X_API_KEY']
    forms = []

    forms_for_user(user).each do |form|
      forms << {
        "Key": form[:key],
        "DisplayName": form[:display_name],
        "FeedbackForm": false
      }
    end

    forms.to_json
  end

  get "/published/:id" do
    user = request.env['HTTP_X_API_KEY']
    form = @database[:forms].where(username: user, key: params['id']).first

    if form.nil?
      response.status = 404
      return {}.to_json
    end

    {
      id: form[:key],
      values: form[:form]
    }.to_json
  end

  get "/seed/:user" do
    seed_data_for_user(params['user'])

    @database[:forms].where(username: params['user']).select(:key).all.to_json
  end

  private

  def forms_for_user(user)
    forms = @database[:forms].where(username: user).all

    if forms.empty?
      seed_data_for_user(user)
      return @database[:forms].where(username: user).all
    end

    forms
  end

  def seed_data_for_user(user)
    forms = Dir.entries("./example_forms").select { |filename| File.file?("./example_forms/#{filename}") }
    forms.map do |filename|
      File.open("./example_forms/#{filename}") do |f|
        file_content = f.read
        if @database[:forms].where(username: user).where(key: filename).all.count == 0
          @database[:forms].insert(
            username: user,
            key: filename,
            display_name: filename,
            form: file_content
          )
        end
      end
    end
  end
end
