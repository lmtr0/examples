import { Component } from 'preact';
import styles from './style.module.css';


const About = ({ query }) => (
	<section class={styles.about}>
		<h1>About</h1>
		<p>A page all about this website.</p>
		<pre>{JSON.stringify(query)}</pre>
	</section>
);

export default About;

export interface HelloWorldProps {
    name: string
}

export class HelloWorld extends Component<HelloWorldProps, any> {
    render (props: HelloWorldProps) {
        return <p>Hello {props.name}!</p>
    }
}