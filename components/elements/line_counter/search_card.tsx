import {Button, Card, Col, Container, Row} from "react-bootstrap";
import 'bootstrap/dist/css/bootstrap.min.css';
import React from "react";
import styles from "styles/components/elements/line_counter/search_card.module.css"

type Props = {

}

class SearchCard extends React.Component<Props, {}> {
	render(): React.ReactNode {
		return (
			<Container>
				<Row>
					<Col lg={10}>
						<Card className={`${styles.search_card}`}>
							<Row className={`${styles.input_field}`}>
								<Col xs={{span: 8, offset: 1}}>
									<Card body >please select path.</Card>
								</Col>
								<Col xs={2}>
									<Button variant="outline-danger" type='button'>Delete</Button>
								</Col>
							</Row>
							<Row>
								<Col xs={{span: 8, offset: 1}}>
									<Card body >please select path.</Card>
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
