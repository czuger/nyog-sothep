require 'simplecov'

SimpleCov.start 'rails' do
  add_filter '/mailers/'
  add_filter '/channels/'
  add_filter '/jobs/'
end

ENV['RAILS_ENV'] ||= 'test'
require File.expand_path('../../config/environment', __FILE__)
require 'rails/test_help'

class ActiveSupport::TestCase
  # Setup all fixtures in test/fixtures/*.yml for all tests in alphabetical order.
  fixtures :all

  # Add more helper methods to be used by all tests here...
  include FactoryGirl::Syntax::Methods
end

require 'mocha/mini_test'
