require "sequel"

class Migrator
  def initialize
    Sequel.extension :migration
  end

  def destroy(database)
    Sequel::Migrator.run(database, "#{__dir__}/migrations", target: 0)
  end

  def migrate(database)
    Sequel::Migrator.run(database, "#{__dir__}/migrations")
  end

  def migrate_to(database, version)
    Sequel::Migrator.run(database, "#{__dir__}/migrations", target: version)
  end
end

class Database
  def initialize
    @migrator = Migrator.new
  end

  def connect
    database = Sequel.connect(ENV['DATABASE_URL'])
    load_extensions_for(database)

    @migrator.migrate(database)
    database
  end

  private

  def load_extensions_for(database)
    database.extension :pg_json
    database.extension :pg_array
  end
end
