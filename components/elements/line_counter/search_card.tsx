import {Button, Card, Col, Container, Form, InputGroup, Row} from "react-bootstrap";
import 'bootstrap/dist/css/bootstrap.min.css';
import React from "react";
import styles from "styles/components/elements/line_counter/search_card.module.css"

type Props = {

}

class SearchCard extends React.Component<Props, {}> {
	render(): React.ReactNode {
		return (
			<Container>
				<Row className="justify-content-md-center">
					<Col xs={10}>
						<Card className={`${styles.search_card}`}>
							<Row className={`${styles.input_field}`}>
								<Col xs={{span: 10, offset: 0}}>
									<InputGroup>
										<InputGroup.Text>Start Text</InputGroup.Text>
											<Form.Control placeholder="text or regular expression"/>
									</InputGroup>
								</Col>
								<Col xs={{span: 1, offset: 0}}>
									<Button variant="danger" type='button'>Delete</Button>
								</Col>
							</Row>
							<Row>
								<Col xs={{span: 10, offset: 0}}>
									<InputGroup>
										<InputGroup.Text>End Text</InputGroup.Text>
											<Form.Control placeholder="text or regular expression"/>
									</InputGroup>
								</Col>
							</Row>
						</Card>
					</Col>
				</Row>
			</Container>
		);
	}

}

export default SearchCard;
