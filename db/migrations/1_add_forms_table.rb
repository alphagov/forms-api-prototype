Sequel.migration do
  change do
    create_table :forms do
      primary_key :id, type: :Bignum
      String :username
      String :key
      String :display_name
      column :form, 'json'
    end
  end
end
