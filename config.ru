RACK_ENV = ENV['RACK_ENV'] ||= 'development' unless defined?(RACK_ENV)
require_relative 'loader'
require_relative './server'

run Server
