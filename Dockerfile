FROM ruby:2-alpine

# Update system
RUN apk upgrade --no-cache

# Prepare app environment
RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

# Install ruby dependencies
COPY .bundle/ /usr/src/app/.bundle/
COPY Gemfile /usr/src/app/
COPY Gemfile.lock /usr/src/app/
RUN set -x \
  && apk add --no-cache --virtual .bundle-deps \
    build-base \
    libxml2-dev \
    libxslt-dev \
  # Throw errors if Gemfile has been modified since Gemfile.lock
  && bundle config --global frozen 1 \
  && bundle config build.nokogiri \
    --use-system-libraries \
    --with-xml2-config=/usr/bin/xml2-config \
    --with-xslt-config=/usr/bin/xslt-config \
  && bundle install --no-cache \
  && apk del .bundle-deps

# Install shared object dependencies
RUN apk add --no-cache libxslt

# Copy source codes
COPY . /usr/src/app

CMD ["bundle", "exec", "./run", "--production"]
