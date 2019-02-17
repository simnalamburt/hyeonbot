# Copyright 2015-2019 Hyeon Kim
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

#
# Build phase
#
FROM ruby:2-alpine
WORKDIR /tmp
COPY Gemfile .
COPY Gemfile.lock .

# Install build dependencies for native extensions
RUN apk add --no-cache --virtual .bundle-deps \
      build-base \
      libxml2-dev \
      libxslt-dev \
      sqlite-dev

# Install gems
RUN gem update bundler
RUN bundle config --global frozen 1
RUN bundle config build.nokogiri \
      --use-system-libraries \
      --with-xml2-config=/usr/bin/xml2-config \
      --with-xslt-config=/usr/bin/xslt-config
RUN bundle install --no-cache

#
# Run phase
#
FROM ruby:2-alpine

# Install shared object dependencies
RUN apk add --no-cache libxslt sqlite-libs
# Copy dependencies
COPY --from=0 /usr/local/bundle /usr/local/bundle
# Copy source codes
WORKDIR /a
COPY run /usr/local/bin/run

CMD ["/usr/local/bin/run"]
