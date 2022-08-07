using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using GraphQlDemo.Data;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

using GraphQlDemo.Queries;
using GraphQlDemo.Repository;
using HotChocolate;


namespace GraphQlDemo
{
    public class Startup
    {
        // This method gets called by the runtime. Use this method to add services to the container.
        // For more information on how to configure your application, visit https://go.microsoft.com/fwlink/?LinkID=398940
        public void ConfigureServices(IServiceCollection services)
        {
            // A better way to make this is to create repositories with methods that will be use instead of interacting direct with the database
            services.AddHttpContextAccessor();
            services.AddSingleton<BookRepository>();
            services
                .AddGraphQLServer()
                //.AddType<Book>()
                .AddQueryType<Query>()
                .AddMutationType<Mutations>();
        }

        // This method gets called by the runtime. Use this method to configure the HTTP request pipeline.
        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            app.UseHttpsRedirection();

            // goto https://localhost:5001/ui/altair
            app.UseGraphQLAltair();
            
            //app.UseGraphQL();
            
            app
                .UseRouting()
                .UseEndpoints(endpoints =>
                {
                    endpoints.MapGraphQL();
                });

        }
    }
}
